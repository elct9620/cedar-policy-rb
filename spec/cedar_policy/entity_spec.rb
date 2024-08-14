# frozen_string_literal: true

RSpec.describe CedarPolicy::Entity do
  let(:uid) { CedarPolicy::EntityUid.new("User", 1) }
  subject(:entity) { CedarPolicy::Entity.new(uid) }

  it { is_expected.to have_attributes(uid: CedarPolicy::EntityUid.new("User", 1)) }
  it { is_expected.to have_attributes(attrs: {}) }
  it { is_expected.to have_attributes(parents: []) }

  describe "with invalid euid" do
    let(:uid) { "1" }

    it { expect { entity }.to raise_error(ArgumentError) }
  end

  describe "#to_hash" do
    subject { entity.to_hash }

    it { is_expected.to include(uid: CedarPolicy::EntityUid.new("User", 1)) }
    it { is_expected.to include(attrs: {}) }
    it { is_expected.to include(parents: []) }
  end
end
