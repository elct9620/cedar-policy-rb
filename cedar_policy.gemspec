# frozen_string_literal: true

require_relative "lib/cedar_policy/version"

Gem::Specification.new do |spec| # rubocop:disable Metrics/BlockLength
  spec.name = "cedar_policy"
  spec.version = CedarPolicy::VERSION
  spec.authors = ["Aotokitsuruya"]
  spec.email = ["contact@aotoki.me"]

  spec.summary = "Ruby bindings for Cedar policy evaluation engine."
  spec.description = "Ruby bindings for Cedar policy evaluation engine."
  spec.homepage = "https://github.com/elct9620/cedar-policy-rb"
  spec.license = "Apache-2.0"
  spec.required_ruby_version = ">= 3.0.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/elct9620/cedar-policy-rb"
  spec.metadata["changelog_uri"] = "https://github.com/elct9620/cedar-policy-rb"

  spec.metadata["rubygems_mfa_required"] = "true"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ rakelib/ .git .github appveyor Gemfile .husky commitlint.config.js
                          package.json package-lock.json])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/cedar_policy/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
