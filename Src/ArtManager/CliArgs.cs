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
        [OptionParameter("cont", 'c')]
        public string Customer { get; set; }

        [OptionParameter("ref", 'r')]
        public string Reference { get; set; }
    }

    /// <summary>
    /// YCH (Your Character Here) is a type of request where the
    /// picture is already known but not the character. Each
    /// character has their own respective slot that someone
    /// can request to have theirs in.
    /// </summary>
    class YchArgs : BaseArgs
    {
        [Required]
        [OptionParameter("slot", 's')]
        public int Slot { get; set; }
    }

    /// <summary>
    /// Raffles are YCHs where the slots are randomly determined
    /// by a random number generator. For this reason, only the
    /// maximum number of slots is required.
    /// </summary>
    class RaffleArgs : BaseArgs
    {
        /// <summary>
        /// Maximum number of slots available
        /// </summary>
        [Required]
        [OptionParameter("max", 'm')]
        public int Max { get; set; }
    }

    class ReqArgs : BaseArgs
    {
        [Required]
        [OptionParameter("desc", 'd')]
        public string Description { get; set; }
    }

    /// <summary>
    /// Commission arguments extend the request by adding
    /// price and payment options.
    /// </summary>
    class ComArgs : ReqArgs
    {
        [Required]
        [OptionParameter("price", 'p')]
        public decimal Price { get; set; }

        [Required]
        [OptionParameter("payment", 'P')]
        public string Payment { get; set; }
    }
}
