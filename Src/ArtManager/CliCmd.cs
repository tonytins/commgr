// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.IO;
using ArtManager.Models;
using EntryPoint;

namespace ArtManager
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
                Price = cli.Price,
                Slot = cli.Slot,
                Ticket = cli.Ticket
            };
            var order = new Order(art);
            order.DBInsert();

            if (cli.Debug)
                order.DbListAll();
        }


        [Command("raf")]
        public void Raffle(string[] args)
        {
            var order = new Order();
            order.DBRaffle(args);
        }
    }
}