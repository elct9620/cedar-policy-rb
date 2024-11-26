# frozen_string_literal: true

RSpec.describe CedarPolicy::Request do
  let(:principal) { CedarPolicy::EntityUid.new("User", "1") }
  let(:action) { CedarPolicy::EntityUid.new("Action", "view") }
  let(:resource) { CedarPolicy::EntityUid.new("Image", "1") }
  let(:ctx) { CedarPolicy::Context.new }

  let(:request) { CedarPolicy::Request.new(principal, action, resource, ctx) }
  let(:entities) { [] }

  let(:policy_set) { CedarPolicy::PolicySet.new(policy) }
  let(:policy) do
    <<~POLICY
      permit(
        principal,
        action,
        resource
      );
    POLICY
  end

  let(:authorizer) { CedarPolicy::Authorizer.new }

  subject { authorizer.authorized?(request, policy_set, entities) }

  it { is_expected.to be_truthy }

  describe "with empty entities" do
    let(:entities) { CedarPolicy::Entities.new }

    it { is_expected.to be_truthy }
  end

  describe "with nested namespace principal" do
    let(:principal) { CedarPolicy::EntityUid.new("Admin::User", "1") }

    it { is_expected.to be_truthy }
  end

  describe "with invalid namespace principal" do
    let(:principal) { CedarPolicy::EntityUid.new("Admin/User", "1") }

    it { expect { subject }.to raise_error(CedarPolicy::ParseError).with_message(/unexpected token/) }
  end

  describe "with invalid principal" do
    let(:principal) { 1 }

    it { expect { subject }.to raise_error(ArgumentError).with_message(/no implicit conversion/) }
  end

  describe "with uid only entity" do
    let(:policy) do
      <<~POLICY
        permit(
          principal == User::"1",
          action == Action::"view",
          resource == Image::"1"
        );
      POLICY
    end
    let(:entities) do
      CedarPolicy::Entities.new([
                                  CedarPolicy::Entity.new(CedarPolicy::EntityUid.new("Image", "1"))
                                ])
    end

    it { is_expected.to be_truthy }
  end

  describe "with role attribute" do
    let(:policy) do
      <<~POLICY
        permit(
          principal == User::"1",
          action == Action::"view",
          resource
        ) when { principal.role == "admin" };
      POLICY
    end
    let(:entities) do
      CedarPolicy::Entities.new(
        [
          CedarPolicy::Entity.new(
            CedarPolicy::EntityUid.new("User", "1"),
            { role: "admin" }
          )
        ]
      )
    end

    it { is_expected.to be_truthy }
  end

  describe "with role parents" do
    let(:policy) do
      <<~POLICY
        permit(
          principal == User::"1",
          action == Action::"view",
          resource
        ) when { principal in Role::"admin" };
      POLICY
    end
    let(:entities) do
      CedarPolicy::Entities.new(
        [
          CedarPolicy::Entity.new(
            CedarPolicy::EntityUid.new("User", "1"),
            {},
            [CedarPolicy::EntityUid.new("Role", "admin")]
          ),
          CedarPolicy::Entity.new(CedarPolicy::EntityUid.new("Role", "admin"))
        ]
      )
    end

    it { is_expected.to be_truthy }
  end

  describe "with context" do
    let(:policy) do
      <<~POLICY
        permit(
          principal,
          action,
          resource
        ) when { context.mfa_enabled == true };
      POLICY
    end
    let(:ctx) { CedarPolicy::Context.new({ mfa_enabled: true }) }

    it { is_expected.to be_truthy }
  end

  describe "with policy defined" do
    let(:policy) do
      <<~POLICY
        permit(
          principal == AdminUser::"1",
          action == Action::"view",
          resource
        );
      POLICY
    end

    it { is_expected.to be_falsey }
  end

  describe "#authorize" do
    subject { authorizer.authorize(request, policy_set, entities) }

    it { is_expected.to have_attributes(decision: CedarPolicy::Decision::ALLOW) }
    it { is_expected.to have_attributes(diagnostics: be_a(CedarPolicy::Diagnostics)) }
    it { is_expected.to have_attributes(diagnostics: have_attributes(reason: ["policy0"])) }
    it { is_expected.to have_attributes(diagnostics: have_attributes(errors: be_none)) }

    describe "when the policy denies the request" do
      let(:policy) do
        <<~POLICY
          permit(
            principal == AdminUser::"1",
            action == Action::"view",
            resource
          );
        POLICY
      end

      it { is_expected.to have_attributes(decision: CedarPolicy::Decision::DENY) }
      it {
        is_expected.to have_attributes(diagnostics: have_attributes(errors: be_none))
      }
    end

    context "when there is schema attached to entities" do
      subject { authorizer.authorize(request, policy_set, entities) }

      let(:policy) do
        <<~POLICY
          permit(
            principal is User,
            action == Action::"View",
            resource
          );

          permit(
            principal is User,
            action == Action::"Edit",
            resource
          ) when { resource.owner == principal };

          permit(
            principal,
            action in Action::"AdminActions",
            resource
          ) when { principal has isAdmin && principal.isAdmin };
        POLICY
      end

      let(:schema) do
        <<~SCHEMA
          entity User {
            isAdmin?: Bool
          };

          entity Image {
            owner: User
          };

          action AdminActions;

          action View in [AdminActions] appliesTo {
              principal: [User],
              resource: [Image]
          };

          action Edit in [AdminActions] appliesTo {
              principal: [User],
              resource: [Image]
          };

          action Delete in [AdminActions] appliesTo {
              principal: [User],
              resource: [Image]
          };
        SCHEMA
      end

      let(:entities) do
        CedarPolicy::Entities.new(entities_data, schema: schema)
      end

      describe "User can View image" do
        let(:action) { CedarPolicy::EntityUid.new("Action", "View") }
        let(:entities_data) {
          [
            {uid: {type: "User", id: 1}, attrs: {}, parents: []},
            {uid: {type: "Image", id: 1}, attrs: {owner: {type: "User", id: " 1"}}, parents: []}
          ]
        }

        it { is_expected.to(have_attributes(decision: CedarPolicy::Decision::ALLOW)) }
      end

      describe "Owner can Edit image" do
        let(:action) { CedarPolicy::EntityUid.new("Action", "Edit") }
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: "1"}}, parents: []}
          ]
        }

        it { is_expected.to(have_attributes(decision: CedarPolicy::Decision::ALLOW)) }
      end

      describe "Non-Owner cannot Edit image" do
        let(:principal) { CedarPolicy::EntityUid.new("User", "3") }
        let(:action) { CedarPolicy::EntityUid.new("Action", "Edit") }
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {}, parents: []},
            {uid: {type: "User", id: "3"}, attrs: {}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: " 1"}}, parents: []}
          ]
        }

        it { is_expected.to(have_attributes(decision: CedarPolicy::Decision::DENY)) }
      end

      describe "Admin can Delete image" do
        let(:principal) { CedarPolicy::EntityUid.new("User", "2") }
        let(:action) { CedarPolicy::EntityUid.new("Action", "Delete") }
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {}, parents: []},
            {uid: {type: "User", id: "2"}, attrs: {isAdmin: true}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: " 1"}}, parents: []}
          ]
        }

        it { is_expected.to(have_attributes(decision: CedarPolicy::Decision::ALLOW)) }
      end

      describe "entity with superfluous attribute" do
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {extra: "0xfeedc0de"}, parents: []},
            {uid: {type: "User", id: "2"}, attrs: {isAdmin: true}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: " 1"}}, parents: []}
          ]
        }

        it { expect { authorizer.authorize(request, policy_set, entities) }.to(raise_error(ArgumentError)) }
      end

      describe "entity with missing attribute" do
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {}, parents: []},
            {uid: {type: "User", id: "2"}, attrs: {isAdmin: true}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {}, parents: []}
          ]
        }

        it { expect { authorizer.authorize(request, policy_set, entities) }.to(raise_error(ArgumentError)) }
      end

      describe "entity with incorrectly-typed attribute" do
        let(:entities_data) {
          [
            {uid: {type: "User", id: "1"}, attrs: {}, parents: []},
            {uid: {type: "User", id: "2"}, attrs: {isAdmin: "0xfeedc0de"}, parents: []},
            {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: " 1"}}, parents: []}
          ]
        }

        it { expect { authorizer.authorize(request, policy_set, entities) }.to(raise_error(ArgumentError)) }
      end
    end
  end
end
