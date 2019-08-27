/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
using CommandLine;

namespace ArtManager.CLI.Interface
{
    public interface IGlobalArgs
    {
        [Option('D', "debug")]
        bool? Debug { get; set; }

    }

    public interface IBaseArgs : IGlobalArgs
    {
        /// <summary>
        /// Artwork name
        /// </summary>
        [Option('n', "name")]
        string Name { get; set; }

        /// <summary>
        /// Not required for YCH and Raffles.
        /// </summary>
        [Option('d', "desc")]
        string Description { get; set; }

        [Option('r', "ref")]
        string Reference { get; set; }

        /// <summary>
        /// Customer name
        /// </summary>
        [Option('c', "cust")]
        string Customer { get; set; }

        [Option('C', "cont")]
        string Contact { get; set; }

        [Option('p', "price")]
        decimal? Price { get; set; }

        [Option('P', "payment")]
        string Payment { get; set; }

        [Option('t', "tickets")]
        int? Ticket { get; set; }

        [Option('s', "slots")]
        int? Slot { get; set; }
    }
}
