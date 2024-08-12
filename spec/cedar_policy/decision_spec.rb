# frozen_string_literal: true

RSpec.describe CedarPolicy::Decision do
  describe ".allow" do
    subject { CedarPolicy::Decision.allow }

    it { is_expected.to eq(CedarPolicy::Decision.allow) }
    it { is_expected.to be_eql(CedarPolicy::Decision.allow) }
    it { is_expected.to eq(true) }
  end

  describe ".deny" do
    subject { CedarPolicy::Decision.deny }

    it { is_expected.to eq(CedarPolicy::Decision.deny) }
    it { is_expected.to be_eql(CedarPolicy::Decision.deny) }
    it { is_expected.to eq(false) }
    it { is_expected.to eq(nil) }
  end
end
