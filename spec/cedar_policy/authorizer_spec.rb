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
    let(:principal) { CedarPolicy::EntityUid.new("User::Admin", "1") }

    it { is_expected.to be_truthy }
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
  end
end
