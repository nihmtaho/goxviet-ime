#!/usr/bin/env ruby

# Script to add RustBridge.swift to VietnameseIMEFast Xcode project
# Usage: ruby add_rustbridge.rb

require 'xcodeproj'

PROJECT_PATH = 'VietnameseIMEFast.xcodeproj'
FILE_TO_ADD = 'VietnameseIMEFast/RustBridge.swift'

puts "🔧 Adding RustBridge.swift to Xcode project..."
puts ""

# Check if xcodeproj gem is installed
begin
  require 'xcodeproj'
rescue LoadError
  puts "❌ Error: xcodeproj gem not found"
  puts ""
  puts "Please install it with:"
  puts "  gem install xcodeproj"
  puts ""
  puts "If you don't have permission, try:"
  puts "  sudo gem install xcodeproj"
  exit 1
end

# Check if project exists
unless File.exist?(PROJECT_PATH)
  puts "❌ Error: #{PROJECT_PATH} not found"
  puts "Make sure you're running this script from the VietnameseIMEFast directory"
  exit 1
end

# Check if file exists
unless File.exist?(FILE_TO_ADD)
  puts "❌ Error: #{FILE_TO_ADD} not found"
  puts "Make sure RustBridge.swift exists in the VietnameseIMEFast folder"
  exit 1
end

puts "✅ Found project: #{PROJECT_PATH}"
puts "✅ Found file: #{FILE_TO_ADD}"
puts ""

# Open project
puts "📂 Opening Xcode project..."
project = Xcodeproj::Project.open(PROJECT_PATH)

# Find the main group
main_group = project.main_group.find_subpath('VietnameseIMEFast', true)

unless main_group
  puts "❌ Error: Could not find VietnameseIMEFast group in project"
  exit 1
end

puts "✅ Found VietnameseIMEFast group"

# Check if file is already in project
existing_file = main_group.files.find { |f| f.path == 'RustBridge.swift' }

if existing_file
  puts "⚠️  Warning: RustBridge.swift is already in the project"
  puts ""
  print "Do you want to remove and re-add it? (y/n): "
  response = gets.chomp.downcase
  
  if response == 'y'
    puts "🗑️  Removing existing reference..."
    existing_file.remove_from_project
  else
    puts "❌ Aborted"
    exit 0
  end
end

# Add file reference
puts "➕ Adding RustBridge.swift to project..."
file_ref = main_group.new_reference('RustBridge.swift')

# Find the main target
target = project.targets.find { |t| t.name == 'VietnameseIMEFast' }

unless target
  puts "❌ Error: Could not find VietnameseIMEFast target"
  exit 1
end

puts "✅ Found target: #{target.name}"

# Add file to compile sources build phase
puts "🔨 Adding to compile sources..."
target.add_file_references([file_ref])

# Save project
puts "💾 Saving project..."
project.save

puts ""
puts "✅ Success! RustBridge.swift has been added to the Xcode project"
puts ""
puts "Next steps:"
puts "  1. Open Xcode: open #{PROJECT_PATH}"
puts "  2. Verify RustBridge.swift appears in Project Navigator"
puts "  3. Build project: Cmd+B"
puts "  4. Run app: Cmd+R"
puts ""
puts "If you encounter any issues, try:"
puts "  • Clean Build Folder (Cmd+Shift+K)"
puts "  • Delete DerivedData: rm -rf ~/Library/Developer/Xcode/DerivedData"
puts "  • Restart Xcode"
puts ""