// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;

namespace Art_Manager
{
    class CliArgs : BaseCliArguments
    {
        public CliArgs() : base("Art Manager") { }

        [Option("debug", 'D')]
        public bool Debug { get; set; }

        [Option("name", 'n')]
        public string Name { get; set; }

        [Option("desc", 'd')]
        public string Description { get; set; }

        [Option("cont", 'c')]
        public string Customer { get; set; }

        [Option("price", 'p')]
        public decimal Price { get; set; }

        [Option("payment", 'P')]
        public string Payment { get; set; }

        [Option("reference", 'r')]
        public string Reference { get; set; }
    }
}
