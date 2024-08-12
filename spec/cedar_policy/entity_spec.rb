# frozen_string_literal: true

RSpec.describe CedarPolicy::EntityUid do
  let(:uid) { CedarPolicy::EntityUid.new("User", "1") }

  subject(:entity) { CedarPolicy::Entity.new(uid) }

  describe "#uid" do
    subject { entity.uid }

    it { is_expected.to eq(uid) }
  end
end
