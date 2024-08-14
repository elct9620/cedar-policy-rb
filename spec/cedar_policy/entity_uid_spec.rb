# frozen_string_literal: true

RSpec.describe CedarPolicy::EntityUid do
  subject(:uid) { CedarPolicy::EntityUid.new("User", 1) }

  describe "#to_s" do
    subject { uid.to_s }

    it { is_expected.to eq('User::"1"') }
  end

  describe "#to_hash" do
    subject { uid.to_hash }

    it { is_expected.to include(type: "User") }
    it { is_expected.to include(id: "1") }
  end
end
