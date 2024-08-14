# frozen_string_literal: true

RSpec.describe CedarPolicy::Decision do
  it { expect { CedarPolicy::Decision.new }.to raise_error(TypeError).with_message(/allocator undefined/) }

  describe "ALLOW" do
    subject { CedarPolicy::Decision::ALLOW }

    it { is_expected.to eq(true) }
  end

  describe "DENY" do
    subject { CedarPolicy::Decision::DENY }

    it { is_expected.to eq(false) }
    it { is_expected.to eq(nil) }
  end
end
