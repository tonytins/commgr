// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using ArtManager.CLI.Interface;
using ArtManager.Models;
using CommandLine;

namespace ArtManager.CLI.Commands
{
    [Verb("list")]
    class ListOpt : ICommand, IBaseArgs
    {
        readonly Order _order;

        public string Name { get; set; }
        public string Description { get; set; }
        public string Reference { get; set; }
        public string Customer { get; set; }
        public string Contact { get; set; }
        public decimal? Price { get; set; }
        public string Payment { get; set; }
        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public bool? Debug { get; set; }

        public int RunCommand(IBaseArgs cli)
        {
            _order.DbListAll();

            return Environment.ExitCode;
        }
    }
}