// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;

namespace ArtManager
{
    class BaseArgs : BaseCliArguments
    {
        public BaseArgs() : base(ArtmConsts.PROGNAME) { }

        [Option("debug", 'D')]
        public bool Debug { get; set; }

        /// <summary>
        /// Artwork name
        /// </summary>
        [Required]
        [OptionParameter("name", 'n')]
        public string Name { get; set; }

        /// <summary>
        /// Customer name
        /// </summary>
        [Required]
        [OptionParameter("cust", 'c')]
        public string Customer { get; set; }

        [Required]
        [OptionParameter("cont", 'C')]
        public string Contact { get; set; }

        /// <summary>
        /// Not required for YCH and Raffles.
        /// </summary>
        [OptionParameter("desc", 'd')]
        public string Description { get; set; }

        [OptionParameter("ref", 'r')]
        public string Reference { get; set; }
    }

    /// <summary>
    /// YCH (Your Character Here) is a type of request where the
    /// picture is already known but not the character. Each
    /// character has their own respective slot that someone
    /// can request to have theirs in.
    /// </summary>
    class YchArgs : PayArgs
    {
        [Required]
        [OptionParameter("ticket", 't')]
        public int Ticket { get; set; }

        [Required]
        [OptionParameter("slot", 's')]
        public int Slot { get; set; }
    }

    /// <summary>
    /// Commission arguments extend the request by adding
    /// price and payment options.
    /// </summary>
    class PayArgs : BaseArgs
    {
        [OptionParameter("price", 'p')]
        public decimal Price { get; set; }

        [Required]
        [OptionParameter("payment", 'P')]
        public string Payment { get; set; }
    }
}
