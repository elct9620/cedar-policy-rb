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

### PolicySet

Define a policy by Cedar Language:

```ruby
policy = <<~POLICY
          permit(
            principal == AdminUser::"1",
            action == Action::"view",
            resource
          );
        POLICY
policy_set = CedarPolicy::PolicySet.new(policy)
```

> Currently, the single policy is not supported.

### Schema

Optionally, define a schema in Cedar Schema Language:

```ruby
schema = CedarPolicy::Schema.new(
  <<~SCHEMA
    entity User, Admin, Image;

    action view appliesTo {
        principal: [User],
        resource: [Image]
    };

    action delete appliesTo {
        principal: [Admin],
        resource: [Image]
    };
  SCHEMA
)
```

You can check that your schema parsed as expected with the `#principals`, `#resources`, `#action_groups`, and `#actions` methods on `Schema`.

### Request

Prepare the Entity's ID via `EntityUid` or an object with `#to_hash` method which returns a hash with `:type` and `:id` keys.

```ruby
principal = CedarPolicy::EntityUid.new("User", "1") # or { type: "User", id: "1" }
action = CedarPolicy::EntityUid.new("Action", "view")
resource = CedarPolicy::EntityUid.new("Image", "1")
```

The `Context` object is used to store the request context. Use `Context` or an object with `#to_hash` method which returns a hash.

```ruby
ctx = CedarPolicy::Context.new({ ip: "127.0.0.1" }) # or { ip: "127.0.0.1" }
```
> The `Context` object can initialize without any arguments as an empty context.

Create a `Request` object with the principal, action, resource, and context.

```ruby
request = CedarPolicy::Request.new(principal, action, resource, ctx)
```

### Entities

Define the entities with related this request. It should be an array of `Entity` objects which have `#to_hash` method returns a hash with `:uid`,`:attrs`, and `:parents` keys.

```ruby
entities = CedarPolicy::Entities.new([
    CedarPolicy::Entity.new(
        CedarPolicy::EntityUid.new("User", "1"),
        { role: "admin" },
        [] # Parents' EntityUid
    ),
    {
        uid: { type: "Image", id: "1" },
        attrs: {},
        parents: []
    }
])
```

You can optionally pass a CedarPolicy::Schema to `Entities.new`, which will allow Cedar to evaluate action groups and validate the structure of your entities:

```ruby
entities = CedarPolicy::Entities.new(entities_array, schema: schema)
```

> Entities will not be validated until they are used for authorization.

### Authorizer

Create an `Authorizer` object and authorize the request with the policy set and entities.

```ruby
authorizer = CedarPolicy::Authorizer.new
```

If boolean result is enough, use `#authorize?` method.

```ruby
authorizer.authorize?(request, policy_set, entities) # => true
```

If you want to get the decision object, use `#authorize` method.

```ruby
response = authorizer.authorize(request, policy_set, entities)
response.decision # => CedarPolicy::Decision::ALLOW
```

> The diagnostics is not supported yet in the response.

## Roadmap

* [ ] Add DSL to improve developer experience
* [ ] Add batch authorization support
* [x] Diagnostics return with response
* [ ] Validator support
* [ ] Schema support

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/elct9620/cedar-policy-rb.

## License

The gem is available as open source under the terms of the [Apache-2.0 License](https://opensource.org/license/apache-2-0).
