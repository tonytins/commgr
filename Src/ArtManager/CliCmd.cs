// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Threading.Tasks;
using EntryPoint;

namespace ArtManager
{
    class CliCmd : BaseCliCommands
    {
        readonly string _dbDir = $"{Environment.CurrentDirectory}\\db";

        [DefaultCommand]
        [Command("req")]
        public async Task Request(string[] args)
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

            await order.JsonFileAsync($"{_dbDir}\\{cli.Name}.arty");
        }

        [Command("com")]
        public async Task Commission(string[] args)
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

            await order.JsonFileAsync($"{_dbDir}\\{cli.Name}.artc");
        }

        [Command("ych")]
        public async Task YCH(string[] args)
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

            await order.JsonFileAsync($"{_dbDir}\\{cli.Name}-{cli.Ticket}-{cli.Slot}.arty");
        }

        /*
        [Command("raf")]
        public async Task Raffle(string[] args)
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

            await order.JsonFileAsync($"{Environment.CurrentDirectory}\\{cli.Name}.arty");
        }
        */
    }
}