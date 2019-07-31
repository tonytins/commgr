using System;
using System.Diagnostics;
using ArtManager.Models;
using Xunit;
using Bogus;

namespace ArtManager.Tests
{
    public class HashTests
    {
        const string VERSION = "0.2";
        const string ORDER_TYPE = "order type:";

        Art _art;

        public HashTests()
        {
            Randomizer.Seed = new Random(8675309);
        }

        Faker<Art> GenArt
        {
            get
            {
                var artData = new Faker<Art>()
                    .RuleFor(o => o.Name, r => r.Lorem.Word())
                    .RuleFor(o => o.Description, r => r.Lorem.Paragraph())
                    .RuleFor(o => o.Price, r => r.Finance.Amount(1, 100))
                    .RuleFor(o => o.Status, r => r.Finance.TransactionType())
                    .RuleFor(o => o.Ticket, r => r.Random.Int(1, 20))
                    .RuleFor(o => o.Slot, r => r.Random.Int(1, 10));

                return artData;
            }
        }

        Faker<Customer> GenCust
        {
            get
            {
                var custData = new Faker<Customer>()
                    .RuleFor(o => o.Name, r => r.Name.FullName())
                    .RuleFor(o => o.Payment, r => r.Finance.BitcoinAddress())
                    .RuleFor(o => o.Contact, r => r.Internet.Email());

                return custData;
            }
        }


        [Fact]
        public void TestRequest()
        {
            var gArt = GenArt.Generate();
            var gCust = GenCust.Generate();
            _art = new Art()
            {
                Name = gArt.Name,
                Description = gArt.Description,
                Custmer = new Customer()
                {
                    Name = gCust.Name,
                    Contact = gCust.Contact
                }
            };

            TestUtils.VersionCheck(_art.Version, VERSION);

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(gArt.Name, gCust.Name, gCust.Contact));
            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}");
            Debug.WriteLine(ArtUtils.AsJson(_art));
        }

        [Fact]
        public void TestCommission()
        {
            var gArt = GenArt.Generate();
            var gCust = GenCust.Generate();
            _art = new Art()
            {
                Name = gArt.Name,
                Description = gArt.Description,
                Status = gArt.Status,
                Custmer = new Customer()
                {
                    Name = gCust.Name,
                    Contact = gCust.Contact,
                    Payment = gCust.Payment
                },
                Price = gArt.Price,
            };

            TestUtils.VersionCheck(_art.Version, VERSION);

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(gArt.Name, gArt.Price.Value, gCust.Name, gCust.Contact));
            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}");
            Debug.WriteLine(ArtUtils.AsJson(_art));
        }

        [Fact]
        public void TestYCH()
        {
            var gArt = GenArt.Generate();
            _art = new Art()
            {
                Name = gArt.Name,
                Status = gArt.Status,
                Price = gArt.Price,
                Ticket = gArt.Ticket,
                Slot = gArt.Slot,
            };

            TestUtils.VersionCheck(_art.Version, VERSION);

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(gArt.Name, gArt.Price.Value, gArt.Ticket.Value, gArt.Slot.Value));
            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}");
            Debug.WriteLine(ArtUtils.AsJson(_art));
        }
    }
}
