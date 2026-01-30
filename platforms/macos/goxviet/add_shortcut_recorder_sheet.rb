#!/usr/bin/env ruby

# Script to add ShortcutRecorderSheet.swift to goxviet Xcode project
# Usage: ruby add_shortcut_recorder_sheet.rb

require 'xcodeproj'

PROJECT_PATH = 'goxviet.xcodeproj'
FILE_TO_ADD = 'goxviet/UI/Settings/Components/ShortcutRecorderSheet.swift'
TARGET_NAME = 'goxviet'

puts "🔧 Adding ShortcutRecorderSheet.swift to Xcode project..."
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
  puts "Current directory: #{Dir.pwd}"
  exit 1
end

# Check if file exists
unless File.exist?(FILE_TO_ADD)
  puts "❌ Error: #{FILE_TO_ADD} not found"
  puts "Current directory: #{Dir.pwd}"
  exit 1
end

# Open project
project = Xcodeproj::Project.open(PROJECT_PATH)

# Get main target
target = project.targets.find { |t| t.name == TARGET_NAME }
unless target
  puts "❌ Error: Target '#{TARGET_NAME}' not found"
  puts "Available targets: #{project.targets.map(&:name).join(', ')}"
  exit 1
end

# Find or create Components group
main_group = project.main_group
goxviet_group = main_group.groups.find { |g| g.path == 'goxviet' } || main_group.new_group('goxviet')
ui_group = goxviet_group.groups.find { |g| g.path == 'UI' } || goxviet_group.new_group('UI')
settings_group = ui_group.groups.find { |g| g.path == 'Settings' } || ui_group.new_group('Settings')
components_group = settings_group.groups.find { |g| g.path == 'Components' } || settings_group.new_group('Components')

# Add file reference
file_ref = components_group.new_file(FILE_TO_ADD)

# Add to build phase
target.source_build_phase.add_file_reference(file_ref)

# Save project
project.save

puts "✅ Successfully added ShortcutRecorderSheet.swift to project!"
puts ""
puts "File added to:"
puts "  - Group: goxviet/UI/Settings/Components"
puts "  - Target: #{TARGET_NAME}"
puts "  - Build Phase: Compile Sources"
puts ""
puts "You can now build the project with Xcode or xcodebuild."
