#!/usr/bin/env python3
"""
Script để thêm Swift files vào Xcode project (project.pbxproj)
Được thiết kế đặc biệt cho GoxViet Phase 2 integration.
"""

import os
import sys
import uuid
import re
from pathlib import Path

def generate_uuid():
    """Generate UUID tương thích với Xcode format (24 ký tự hex)"""
    return uuid.uuid4().hex[:24].upper()

def find_section(content, section_name):
    """Tìm vị trí section trong pbxproj"""
    pattern = rf"/\* Begin {section_name} section \*/"
    match = re.search(pattern, content)
    if match:
        return match.end()
    return None

def find_section_end(content, start_pos):
    """Tìm vị trí kết thúc section"""
    pattern = r"/\* End .+ section \*/"
    match = re.search(pattern, content[start_pos:])
    if match:
        return start_pos + match.start()
    return None

def add_file_reference(content, file_path, file_uuid, file_name):
    """Thêm PBXFileReference entry"""
    section_start = find_section(content, "PBXFileReference")
    if not section_start:
        print(f"❌ Không tìm thấy PBXFileReference section")
        return content
    
    section_end = find_section_end(content, section_start)
    if not section_end:
        print(f"❌ Không tìm thấy kết thúc của PBXFileReference section")
        return content
    
    file_type = "sourcecode.swift" if file_path.endswith(".swift") else "text"
    
    entry = f"\t\t{file_uuid} /* {file_name} */ = {{isa = PBXFileReference; lastKnownFileType = {file_type}; path = {file_name}; sourceTree = \"<group>\"; }};\n"
    
    # Insert trước dòng "/* End PBXFileReference section */"
    return content[:section_end] + entry + content[section_end:]

def add_build_file(content, file_uuid, build_uuid, file_name):
    """Thêm PBXBuildFile entry"""
    section_start = find_section(content, "PBXBuildFile")
    if not section_start:
        print(f"❌ Không tìm thấy PBXBuildFile section")
        return content
    
    section_end = find_section_end(content, section_start)
    if not section_end:
        return content
    
    entry = f"\t\t{build_uuid} /* {file_name} in Sources */ = {{isa = PBXBuildFile; fileRef = {file_uuid} /* {file_name} */; }};\n"
    
    return content[:section_end] + entry + content[section_end:]

def add_to_group(content, file_uuid, file_name, group_name):
    """Thêm file vào PBXGroup"""
    # Tìm group theo tên
    pattern = rf'{group_name} = \{{[^}}]+children = \([^)]+\)'
    match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        print(f"⚠️  Không tìm thấy group '{group_name}', sẽ thêm vào root group")
        # Fallback: thêm vào root group (main group)
        pattern = r'mainGroup = ([A-F0-9]{24})'
        match = re.search(pattern, content)
        if match:
            main_group_id = match.group(1)
            pattern = rf'{main_group_id} = \{{[^}}]+children = \([^)]+\)'
            match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        print(f"❌ Không thể thêm file vào group")
        return content
    
    group_section = match.group(0)
    
    # Tìm vị trí đóng ngoặc của children array
    children_end = group_section.rfind(')')
    
    # Thêm file reference vào children array
    new_child = f"\n\t\t\t\t{file_uuid} /* {file_name} */,"
    
    new_group_section = group_section[:children_end] + new_child + group_section[children_end:]
    
    return content.replace(group_section, new_group_section)

def add_to_sources_build_phase(content, build_uuid, file_name, target_name="goxviet"):
    """Thêm file vào PBXSourcesBuildPhase"""
    # Tìm target
    pattern = rf'{target_name} \*/.*?buildPhases = \([^)]+\)'
    match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        print(f"⚠️  Không tìm thấy target '{target_name}'")
        return content
    
    # Tìm PBXSourcesBuildPhase
    pattern = r'([A-F0-9]{24}) /\* Sources \*/.*?\1 /\* Sources \*/ = \{[^}]+files = \([^)]+\)'
    match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        print(f"❌ Không tìm thấy Sources build phase")
        return content
    
    sources_section = match.group(0)
    
    # Tìm vị trí đóng ngoặc của files array
    files_end = sources_section.rfind(')')
    
    # Thêm build file vào files array
    new_file = f"\n\t\t\t\t{build_uuid} /* {file_name} in Sources */,"
    
    new_sources_section = sources_section[:files_end] + new_file + sources_section[files_end:]
    
    return content.replace(sources_section, new_sources_section)

def add_file_to_project(pbxproj_path, file_path, group_name="goxviet", target_name="goxviet"):
    """
    Thêm một file Swift vào Xcode project
    
    Args:
        pbxproj_path: Đường dẫn tới file project.pbxproj
        file_path: Đường dẫn relative từ thư mục gồm pbxproj đến file
        group_name: Tên group trong Xcode (mặc định: goxviet)
        target_name: Tên target (mặc định: goxviet)
    """
    file_name = os.path.basename(file_path)
    
    # Đọc nội dung project file
    with open(pbxproj_path, 'r') as f:
        content = f.read()
    
    # Kiểm tra xem file đã tồn tại chưa
    if file_name in content:
        print(f"⚠️  File '{file_name}' đã tồn tại trong project, bỏ qua...")
        return False
    
    print(f"➕ Thêm file: {file_name}")
    
    # Generate UUIDs
    file_uuid = generate_uuid()
    build_uuid = generate_uuid()
    
    # 1. Thêm PBXFileReference
    content = add_file_reference(content, file_path, file_uuid, file_name)
    
    # 2. Thêm PBXBuildFile (nếu là file Swift)
    if file_path.endswith('.swift'):
        content = add_build_file(content, file_uuid, build_uuid, file_name)
    
    # 3. Thêm vào PBXGroup
    content = add_to_group(content, file_uuid, file_name, group_name)
    
    # 4. Thêm vào Sources build phase (nếu là file Swift)
    if file_path.endswith('.swift'):
        content = add_to_sources_build_phase(content, build_uuid, file_name, target_name)
    
    # Ghi lại file
    with open(pbxproj_path, 'w') as f:
        f.write(content)
    
    return True

def main():
    """Main entry point"""
    # Đường dẫn project
    project_root = Path(__file__).parent.parent
    pbxproj_path = project_root / "platforms/macos/goxviet/goxviet.xcodeproj/project.pbxproj"
    
    if not pbxproj_path.exists():
        print(f"❌ Không tìm thấy file project.pbxproj tại: {pbxproj_path}")
        sys.exit(1)
    
    # Backup project file
    backup_path = str(pbxproj_path) + ".backup"
    import shutil
    shutil.copy2(pbxproj_path, backup_path)
    print(f"📦 Đã tạo backup tại: {backup_path}")
    
    # Danh sách files cần thêm
    files_to_add = [
        # UI Components
        ("goxviet/UI/Shared/GlassBackground.swift", "goxviet"),
        ("goxviet/UI/Settings/Components/SettingRow.swift", "goxviet"),
        ("goxviet/UI/Settings/Components/MetricsChartView.swift", "goxviet"),
        ("goxviet/UI/Settings/GeneralSettingsView.swift", "goxviet"),
        ("goxviet/UI/Settings/PerAppSettingsView.swift", "goxviet"),
        ("goxviet/UI/Settings/AdvancedSettingsView.swift", "goxviet"),
        ("goxviet/UI/Settings/AboutSettingsView.swift", "goxviet"),
        
        # Core
        ("goxviet/Core/RustBridgeError.swift", "goxviet"),
        ("goxviet/Core/RustBridgeSafe.swift", "goxviet"),
        ("goxviet/Core/SettingsManager.swift", "goxviet"),
        ("goxviet/Core/TypedNotifications.swift", "goxviet"),
        
        # Managers
        ("goxviet/Managers/PerAppModeManagerEnhanced.swift", "goxviet"),
        
        # MenuBar
        ("goxviet/UI/MenuBar/SmartModeIndicator.swift", "goxviet"),
        
        # Tests
        ("../goxvietTests/RustBridgeSafeTests.swift", "goxvietTests"),
        ("../goxvietTests/SettingsManagerTests.swift", "goxvietTests"),
        ("../goxvietTests/PerAppModeManagerEnhancedTests.swift", "goxvietTests"),
    ]
    
    added_count = 0
    for file_path, group_name in files_to_add:
        target_name = "goxvietTests" if "Tests" in file_path else "goxviet"
        if add_file_to_project(pbxproj_path, file_path, group_name, target_name):
            added_count += 1
    
    print(f"\n✅ Hoàn tất! Đã thêm {added_count}/{len(files_to_add)} files vào Xcode project")
    print(f"📝 Backup gốc: {backup_path}")
    print(f"\n⚠️  LƯU Ý: Phương pháp này có thể không hoàn hảo.")
    print(f"   Nên mở Xcode và kiểm tra lại project structure.")
    print(f"   Nếu có lỗi, restore từ backup và thêm files thủ công trong Xcode.")

if __name__ == "__main__":
    main()
