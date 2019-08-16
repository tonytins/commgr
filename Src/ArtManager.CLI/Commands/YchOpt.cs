// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using ArtManager.CLI.Interface;
using ArtManager.Models;
using CommandLine;

namespace ArtManager.CLI.Commands
{

    [Verb("ych")]
    class YchOpt : ICommand, IYchArgs
    {
        Art _art;
        Order _order;

        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public decimal? Price { get; set; }
        public string Payment { get; set; }
        public string Customer { get; set; }
        public string Contact { get; set; }
        public string Name { get; set; }
        public string Description { get; set; }
        public string Reference { get; set; }
        public bool Debug { get; set; }

        public int RunCommand(IBaseArgs cli)
        {
            throw new NotImplementedException();
        }

        public int RunCommand(IYchArgs cli)
        {
            _art = new Art()
            {
                Name = cli.Name,
                Customer = new Customer
                {
                    Name = cli.Customer,
                    Contact = cli.Contact,
                    Payment = cli.Payment,
                },
                Price = cli.Price,
                Slot = cli.Slot,
                Ticket = cli.Ticket
            };
            _order = new Order(_art);
            _order.DBInsert();

            if (cli.Debug || Debugger.IsAttached)
                _order.DbListAll();

            return Environment.ExitCode;
        }

        public int RunCommand(ICustArgs cli)
        {
            throw new NotImplementedException();
        }

        public int RunCommand(IPayArgs cli)
        {
            throw new NotImplementedException();
        }
    }

}