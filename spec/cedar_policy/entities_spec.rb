# frozen_string_literal: true

RSpec.describe CedarPolicy::Entities do
  subject(:entities) { CedarPolicy::Entities.new }

  it { is_expected.to be_none }

  describe "have one entity" do
    let(:uid) { CedarPolicy::EntityUid.new("User", 1) }
    let(:entity) { CedarPolicy::Entity.new(uid) }

    subject(:entities) { CedarPolicy::Entities.new([entity]) }

    it { is_expected.to be_one }
  end

  describe "have duplicate entities" do
    let(:uid) { CedarPolicy::EntityUid.new("User", 1) }
    let(:entity) { CedarPolicy::Entity.new(uid) }
    let(:other_entity) { CedarPolicy::Entity.new(uid) }

    subject(:entities) { CedarPolicy::Entities.new([entity, other_entity]) }

    it { is_expected.to be_one }
  end

  describe "with hash entity" do
    let(:entity) { { uid: { type: "User", id: 1 }, attrs: {}, parents: [] } }

    subject(:entities) { CedarPolicy::Entities.new([entity]) }

    it { is_expected.to be_one }

    describe "when euid only" do
      let(:entity) { { uid: { type: "User", id: 1 } } }

      it { is_expected.to be_one }
    end
  end
end
