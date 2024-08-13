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
  ext.cross_compile = true
end

task default: %i[compile spec rubocop]
