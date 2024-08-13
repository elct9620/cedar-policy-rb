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
end
