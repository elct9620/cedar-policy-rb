Cedar Policy
===

Ruby bindings for Cedar policy evaluation engine.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add cedar_policy

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install cedar_policy

## Usage

> [!WARNING]
> This gem is still under development and the API may change in the future.

```ruby
policy = <<~POLICY
          permit(
            principal == AdminUser::"1",
            action == Action::"view",
            resource
          );
        POLICY
policy_set = CedarPolicy::PolicySet.new(policy)

principal = CedarPolicy::EntityUid.new("User", "1")
action = CedarPolicy::EntityUid.new("Action", "view")
resource = CedarPolicy::EntityUid.new("Image", "1")

request = CedarPolicy::Request.new(principal, action, resource)

entities = CedarPolicy::Entities.new([
    CedarPolicy::Entity.new(
        CedarPolicy::EntityUid.new("User", "1"),
        { role: "admin" }
    )
])

authorizer = CedarPolicy::Authorizer.new
authorizer.authorize?(request, policy_set, entities) # => true

response = authorizer.authorize(request, policy_set, entities)
response.decision # => CedarPolicy::Decision::ALLOW
```

## Roadmap

* [ ] Diagnostics return with response
* [ ] Validator support
* [ ] Schema support

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/elct9620/cedar-policy-rb.

## License

The gem is available as open source under the terms of the [Apache-2.0 License](https://opensource.org/license/apache-2-0).
