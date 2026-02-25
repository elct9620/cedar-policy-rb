# frozen_string_literal: true

RSpec.describe CedarPolicy::PolicySet do
  subject { CedarPolicy::PolicySet.new }

  it { is_expected.to be_empty }

  describe "with policy string" do
    subject { CedarPolicy::PolicySet.new(policy_str) }

    context "when the policy string is empty" do
      let(:policy_str) { "" }

      it { is_expected.to be_empty }
    end

    context "when the policy string is invalid" do
      let(:policy_str) { "invalid" }

      it "raises an error" do
        expect { subject }.to raise_error(CedarPolicy::ParseError)
      end
    end

    context "when the policy string is valid" do
      let(:policy_str) do
        <<~POLICY
          permit (
            principal,
            action,
            resource
          );
        POLICY
      end

      it { is_expected.not_to be_empty }
    end
  end

  describe "#policy_ids" do
    context "when the policy set is empty" do
      subject { CedarPolicy::PolicySet.new }

      it "returns an empty array" do
        expect(subject.policy_ids).to eq([])
      end
    end

    context "when the policy set has multiple policies with @id annotations" do
      subject { CedarPolicy::PolicySet.new(policy_str) }

      let(:policy_str) do
        <<~POLICY
          @id("TestPermitPolicy")
          permit (
            principal,
            action,
            resource
          );

          @id("TestForbidPolicy")
          forbid (
            principal,
            action == Action::"delete",
            resource
          );
        POLICY
      end

      it "returns the custom @id annotation values" do
        expect(subject.policy_ids).to contain_exactly("TestPermitPolicy", "TestForbidPolicy")
      end
    end

    context "when the policy set has policies without @id annotations" do
      subject { CedarPolicy::PolicySet.new(policy_str) }

      let(:policy_str) do
        <<~POLICY
          permit (principal, action, resource);
          forbid (principal, action, resource);
        POLICY
      end

      it "falls back to auto-generated policy IDs" do
        expect(subject.policy_ids).to contain_exactly("policy0", "policy1")
      end
    end

    context "when the policy set has mixed @id annotations and auto-generated IDs" do
      subject { CedarPolicy::PolicySet.new(policy_str) }

      let(:policy_str) do
        <<~POLICY
          @id("CustomPolicy")
          permit (principal, action, resource);

          forbid (principal, action, resource);
        POLICY
      end

      it "returns a mix of custom and auto-generated policy IDs" do
        expect(subject.policy_ids).to contain_exactly("CustomPolicy", "policy1")
      end
    end
  end
end
