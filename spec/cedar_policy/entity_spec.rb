# frozen_string_literal: true

RSpec.describe CedarPolicy::EntityUid do
  let(:uid) { CedarPolicy::EntityUid.new("User", "1") }

  subject(:entity) { CedarPolicy::Entity.new(uid) }

  describe "#uid" do
    subject { entity.uid }

    it { is_expected.to eq(uid) }
  end

  describe "with attributes" do
    let(:attributes) { { "name" => '"John Doe"', "age" => "18" } }

    subject(:entity) { CedarPolicy::Entity.new(uid, attributes) }

    it { is_expected.to have_attributes(uid: uid) }

    describe "with invalid expression" do
      let(:attributes) { { "name" => '"John Doe' } }

      it { expect { entity }.to raise_error(CedarPolicy::ParseError).with_message(/invalid token/) }
    end
  end
end
