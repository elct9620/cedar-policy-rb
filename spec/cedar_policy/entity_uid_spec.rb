# frozen_string_literal: true

RSpec.describe CedarPolicy::EntityUid do
  subject(:uid) { CedarPolicy::EntityUid.new("User", 1) }

  describe "#==" do
    let(:other) { CedarPolicy::EntityUid.new("User", 1) }

    it { is_expected.to eq(other) }
  end

  describe "#eql?" do
    let(:other) { CedarPolicy::EntityUid.new("User", 1) }

    it { is_expected.to be_eql(other) }
  end

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
