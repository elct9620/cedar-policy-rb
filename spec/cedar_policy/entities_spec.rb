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

  describe "with schema" do
    let(:schema) do
      CedarPolicy::Schema.new(
        <<~SCHEMA
          entity User {
            isAdmin?: Bool
          };

          entity Image {
            owner: User
          };

          action View  appliesTo {
              principal: [User],
              resource: [Image]
          };
        SCHEMA
      )
    end

    let(:user) { {uid: {type: "User", id: "1"}, attrs: {}, parents: []} }
    let(:admin) { {uid: {type: "User", id: "2"}, attrs: {isAdmin: true}, parents: []} }
    let(:image) { {uid: {type: "Image", id: "1"}, attrs: {owner: {type: "User", id: 1}}, parents: []} }

    subject(:entities) do
      CedarPolicy::Entities.new([user, admin, image], schema: schema)
    end

    it { is_expected.to(satisfy { |entities| entities.to_ary.size == 3 }) }
    it { is_expected.to(have_attributes(schema: schema)) }
  end
end
