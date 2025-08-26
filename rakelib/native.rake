# frozen_string_literal: true

CrossRuby = Struct.new(:version, :platform) do
  # rubocop:disable Lint/ConstantDefinitionInBlock
  DARWIN_PLATFORM_REGEX = /darwin/
  LINUX_PLATFORM_REGEX = /linux/
  WINDOWS_PLATFORM_REGEX = /mingw|mswin/
  # rubocop:enable Lint/ConstantDefinitionInBlock

  def ver
    @ver ||= version[/\A[^-]+/]
  end

  def windows?
    !!(platform =~ WINDOWS_PLATFORM_REGEX)
  end

  def linux?
    !!(platform =~ LINUX_PLATFORM_REGEX)
  end

  def darwin?
    !!(platform =~ DARWIN_PLATFORM_REGEX)
  end
end

CROSS_RUBIES = File.read(".cross_rubies").split("\n").filter_map do |line|
  case line
  when /\A([^#]+):([^#]+)/
    CrossRuby.new(Regexp.last_match(1), Regexp.last_match(2))
  end
end

RUBY_CC_VERSION = CROSS_RUBIES.map(&:ver).uniq.min

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--ruby-versions", RUBY_CC_VERSION, "--build"
end

namespace :gem do
  CROSS_RUBIES.map(&:platform).each do |platform|
    desc "Build native gem for #{platform}"
    task platform do
      sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--ruby-versions", RUBY_CC_VERSION, "--build"
    end
  end

  desc "Build native gems for all platforms"
  multitask "all" => CROSS_RUBIES.map(&:platform).uniq

  desc "Build native gems for windows"
  multitask "windows" => CROSS_RUBIES.select(&:windows?).map(&:platform).uniq

  desc "Build native gems for linux"
  multitask "linux" => CROSS_RUBIES.select(&:linux?).map(&:platform).uniq

  desc "Build native gems for darwin"
  multitask "darwin" => CROSS_RUBIES.select(&:darwin?).map(&:platform).uniq

  desc "Build native gems for all platforms"
  task default: %i[all]
end

desc "Support platform list"
task :platforms, [:format] do |_t, args|
  if args[:format] == "json"
    require "json"
    puts CROSS_RUBIES.map(&:platform).uniq.to_json
  else
    puts CROSS_RUBIES.map(&:platform).uniq
  end
end
