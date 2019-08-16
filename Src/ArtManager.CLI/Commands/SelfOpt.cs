// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using ArtManager.CLI.Interface;
using ArtManager.Models;
using CommandLine;

namespace ArtManager.CLI.Commands
{
    [Verb("self")]
    class SelfOpt : ICommand, IBaseArgs
    {
        Art _art;
        Order _order;

        public string Name { get; set; }
        public string Description { get; set; }
        public string Reference { get; set; }
        public bool Debug { get; set; }

        public int RunCommand(IBaseArgs cli)
        {
            _art = new Art
            {
                Name = cli.Name,
                Description = cli.Description,
                Reference = cli.Reference,
            };
            _order = new Order(_art);
            _order.DBInsert();

            if (cli.Debug)
                ArtUtils.WriteJson(_art);

            return Environment.ExitCode;
        }

        public int RunCommand(IYchArgs cli)
        {
            throw new NotImplementedException();
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