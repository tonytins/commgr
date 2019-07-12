// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using EntryPoint;

namespace ArtManager
{
    class CliCmd : BaseCliCommands
    {
        [DefaultCommand]
        [Command("req")]
        public void Request(string[] args)
        {
            var cli = Cli.Parse<BaseArgs>(args);
            var art = new Art()
            {
                Name = cli.Name,
                Custmer = new Customer
                {
                    Name = cli.Customer,
                    Contact = cli.Contact,
                },
                Description = cli.Description,
            };
            var order = new Order(art);

            if (cli.Debug)
            {
                Console.WriteLine(order.ToJson);
            }
        }

        [Command("com")]
        public void Commission(string[] args)
        {
            var cli = Cli.Parse<PayArgs>(args);
            var art = new Art()
            {
                Name = cli.Name,
                Custmer = new Customer
                {
                    Name = cli.Customer,
                    Contact = cli.Contact,
                    Payment = cli.Payment,
                },
                Price = cli.Price,
                Description = cli.Description,
            };
            var order = new Order(art);

            if (cli.Debug)
            {
                Console.WriteLine(order.ToJson);
            }
        }

        [Command("ych")]
        public void YCH(string[] args)
        {
            var cli = Cli.Parse<YchArgs>(args);
            var art = new Art()
            {
                Name = cli.Name,
                Custmer = new Customer
                {
                    Name = cli.Customer,
                    Contact = cli.Contact,
                    Payment = cli.Payment,
                },
                Price = cli.Price,
                Slot = cli.Slot,
                Ticket = cli.Ticket
            };
            var order = new Order(art);

            if (cli.Debug)
            {
                Console.WriteLine(order.ToJson);
            }
        }

        [Command("raf")]
        public void Raffle(string[] args)
        {
            var cli = Cli.Parse<YchArgs>(args);
            var rand = new Random();
            var slot = rand.Next(cli.Slot);
            var art = new Art()
            {
                Name = cli.Name,
                Custmer = new Customer
                {
                    Name = cli.Customer,
                    Contact = cli.Contact,
                    Payment = cli.Payment,
                },
                Slot = slot,
                Ticket = cli.Ticket
            };
            var order = new Order(art);

            if (cli.Debug)
            {
                Console.WriteLine(order.ToJson);
            }
        }
    }
}