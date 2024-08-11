# frozen_string_literal: true

RSpec.describe CedarPolicy::EntityUid do
  subject(:uid) { CedarPolicy::EntityUid.new("User", "1") }

  describe "with invalid entity type" do
    subject { CedarPolicy::EntityUid.new("1", "1") }

    it { expect { subject }.to raise_error(ArgumentError).with_message(/unexpected token `1`/) }
  end

  describe "#to_s" do
    subject { uid.to_s }

    it { is_expected.to eq('User::"1"') }
  end
end
