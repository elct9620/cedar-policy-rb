# frozen_string_literal: true

RSpec.describe CedarPolicy::Request do
  let(:principal) { CedarPolicy::EntityUid.new("User", "1") }
  let(:action) { CedarPolicy::EntityUid.new("Action", "view") }
  let(:resource) { CedarPolicy::EntityUid.new("Image", "1") }

  subject(:request) { CedarPolicy::Request.new(principal, action, resource, {}) }

  describe "#principal" do
    subject { request.principal }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("User", "1")) }

    describe "with empty principal" do
      let(:principal) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end

  describe "#action" do
    subject { request.action }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("Action", "view")) }

    describe "with empty action" do
      let(:action) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end

  describe "#resource" do
    subject { request.resource }

    it { is_expected.to eq(CedarPolicy::EntityUid.new("Image", 1)) }

    describe "with empty resource" do
      let(:resource) { nil }

      it {
        expect do
          request
        end.to raise_error(ArgumentError).with_message("no implicit conversion of NilClass into EntityUid")
      }
    end
  end
end
