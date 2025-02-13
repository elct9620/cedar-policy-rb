# frozen_string_literal: true

RSpec.describe CedarPolicy::Schema do
  describe "invalid schema raises parse error" do
    subject(:schema) { CedarPolicy::Schema.new("not a valid cedar schema") }

    it { expect { schema }.to(raise_error(CedarPolicy::ParseError)) }
  end

  context("with a valid schema") do
    let(:schema) do
      CedarPolicy::Schema.new(
        <<~SCHEMA
          entity User, Admin, Image;

          action AdminActions;

          action View appliesTo {
              principal: [User],
              resource: [Image]
          };

          action Delete in [AdminActions] appliesTo {
              principal: [Admin],
              resource: [Image]
          };
        SCHEMA
      )
    end

    describe "#actions returns all defined actions and action groups" do
      subject(:actions) { schema.actions }
      it { is_expected.to(contain_exactly(*%w[Action::"AdminActions" Action::"Delete" Action::"View"])) }
    end

    describe "#action_groups returns all defined action groups" do
      subject(:action_groups) { schema.action_groups }
      it { is_expected.to(contain_exactly(*%w[Action::"AdminActions"])) }
    end

    describe "#principals returns all entities mentioned as principals" do
      subject(:principals) { schema.principals }

      it { is_expected.to(contain_exactly(*%w[User Admin])) }
    end

    describe "#resources returns all entities mentioned as resources" do
      subject(:resources) { schema.resources }

      it { is_expected.to(contain_exactly(*%w[Image])) }
    end
  end
end
