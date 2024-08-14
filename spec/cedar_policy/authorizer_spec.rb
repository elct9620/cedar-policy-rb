# frozen_string_literal: true

RSpec.describe CedarPolicy::Request do
  let(:principal) { CedarPolicy::EntityUid.new("User", "1") }
  let(:action) { CedarPolicy::EntityUid.new("Action", "view") }
  let(:resource) { CedarPolicy::EntityUid.new("Image", "1") }

  let(:request) { CedarPolicy::Request.new(principal, action, resource) }
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

  subject(:authorizer) { CedarPolicy::Authorizer.new }

  describe "#authorized?" do
    subject { authorizer.authorized?(request, policy_set, entities) }

    it { is_expected.to be_truthy }

    context "with entities object" do
      let(:entities) { CedarPolicy::Entities.new }

      it { is_expected.to be_truthy }
    end

    context "when the policy denies the request" do
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
  end

  describe "#authorize" do
    subject { authorizer.authorize(request, policy_set, entities) }

    it { is_expected.to have_attributes(decision: CedarPolicy::Decision::ALLOW) }

    context "when the policy denies the request" do
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
    end
  end
end
