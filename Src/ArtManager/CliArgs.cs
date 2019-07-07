// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;

namespace ArtManager
{
    class CliArgs : BaseCliArguments
    {
        public CliArgs() : base("Art Manager") { }

        [Option("debug", 'D')]
        public bool Debug { get; set; }

        [OptionParameter("name", 'n')]
        public string Name { get; set; }

        [OptionParameter("desc", 'd')]
        public string Description { get; set; }

        [OptionParameter("cont", 'c')]
        public string Customer { get; set; }

        [OptionParameter("price", 'p')]
        public decimal Price { get; set; }

        [OptionParameter("payment", 'P')]
        public string Payment { get; set; }

        [OptionParameter("reference", 'r')]
        public string Reference { get; set; }
    }
}
