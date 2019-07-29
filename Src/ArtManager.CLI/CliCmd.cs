// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using System.IO;
using ArtManager.Models;
using EntryPoint;

namespace ArtManager.CLI
{
    class CliCmd : BaseCliCommands
    {
        [DefaultCommand]
        [Command("list")]
        public void ListAll(string[] args)
        {
            if (File.Exists(ArtmConsts.DBFILE))
            {
                var order = new Order();
                order.DbListAll();
            }
            else
            {
                Console.WriteLine(ArtmConsts.DBERR);
            }
        }

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
                Catagory = Catagory.Request,
                Description = cli.Description,
            };
            var order = new Order(art);
            order.DBInsert();

            if (cli.Debug)
                order.DbListAll();
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
                Catagory = Catagory.Commission,
                Price = cli.Price,
                Description = cli.Description,
            };
            var order = new Order(art);
            order.DBInsert();

            if (cli.Debug)
                order.DbListAll();
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
                Catagory = Catagory.YCH,
                Price = cli.Price,
                Slot = cli.Slot,
                Ticket = cli.Ticket
            };
            var order = new Order(art);
            order.DBInsert();

            if (cli.Debug || Debugger.IsAttached)
                order.DbListAll();
        }


        [Command("raf")]
        public void Raffle(string[] args)
        {
            var order = new Order();
            order.DBRaffle(args);
        }

        public override void OnHelpInvoked(string helpText)
        {
            var monero = "44xZM7FSdJ9TpYK99Y2e4JYyprRWR3fKxJWsw4jEFL6CWtWQG35qWAPDTPDuAGy1v74bL2arKP2Eq7GVPfsWTZVs7MhKhf4";
            var about = "A command line application used for storing request, commission, and YCH information.";
            Console.WriteLine($"## About ##{Environment.NewLine}{about}{Environment.NewLine}");
            Console.WriteLine($"{Environment.NewLine}## Donate ##{Environment.NewLine}Ko-Fi: ko-fi.com/antonwilc0x{Environment.NewLine}Monero: {monero}");
            Console.WriteLine($"{Environment.NewLine}{helpText}");
        }
    }
}