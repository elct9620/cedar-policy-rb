# frozen_string_literal: true

RSpec.describe CedarPolicy::Entities do
  subject(:entities) { CedarPolicy::Entities.new(json) }

  let(:json) do
    <<~JSON
      [
       {
         "uid": { "type": "User", "id": "1" },
         "attrs": {},
         "parents": []
       }
      ]
    JSON
  end

  describe "with invalid JSON" do
    let(:json) { "invalid" }

    it { expect { entities }.to raise_error(CedarPolicy::EntitiesError) }
  end

  describe "#get" do
    subject { entities.get(CedarPolicy::EntityUid.new("User", "1")) }

    it { is_expected.to have_attributes(uid: CedarPolicy::EntityUid.new("User", "1")) }

    describe "non-existent entity" do
      subject { entities.get(CedarPolicy::EntityUid.new("User", "2")) }

      it { is_expected.to be_nil }
    end

    describe "with empty entities" do
      let(:entities) { CedarPolicy::Entities.new }

      it { is_expected.to be_nil }
    end
  end
end
