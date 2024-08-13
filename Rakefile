# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("cedar_policy.gemspec")

RbSys::ExtensionTask.new("cedar_policy", GEMSPEC) do |ext|
  ext.lib_dir = "lib/cedar_policy"
end

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--build", "-r", "3.3.0,3.2.0,3.1.0,3.0.0"
end

task default: %i[compile spec rubocop]
