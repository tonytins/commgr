// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using CommandLine;

namespace ArtManager.CLI.Interface
{
    public interface IGlobalArgs
    {
        [Option('D', "debug")]
        bool Debug { get; set; }

    }

    public interface IBaseArgs : IGlobalArgs
    {
        /// <summary>
        /// Artwork name
        /// </summary>
        [Option('n', "name", Required = true)]
        string Name { get; set; }

        /// <summary>
        /// Not required for YCH and Raffles.
        /// </summary>
        [Option('d', "desc")]
        string Description { get; set; }

        [Option('r', "ref")]
        string Reference { get; set; }
    }

    public interface ICustArgs : IBaseArgs
    {
        /// <summary>
        /// Customer name
        /// </summary>
        [Option('c', "cust", Required = true)]
        string Customer { get; set; }

        [Option('C', "cont")]
        string Contact { get; set; }
    }

    /// <summary>
    /// Commission arguments extend the request by adding
    /// price and payment options.
    /// </summary>
    public interface IPayArgs : ICustArgs
    {
        [Option('p', "price")]
        decimal? Price { get; set; }

        [Option('P', "payment")]
        string Payment { get; set; }
    }

    /// <summary>
    /// YCH (Your Character Here) is a type of request where the
    /// picture is already known but not the character. Each
    /// character has their own respective slot that someone
    /// can request to have theirs in.
    /// </summary>
    public interface IYchArgs : IPayArgs
    {
        [Option('t', "tickets")]
        int? Ticket { get; set; }

        [Option('s', "slots")]
        int? Slot { get; set; }
    }

    public interface IRaffleArgs : IGlobalArgs
    {

        /// <summary>
        /// Artwork name
        /// </summary>
        [Option('n', "name")]
        string Name { get; set; }

        [Option('t', "tickets")]
        int? Ticket { get; set; }

        [Option('s', "slots")]
        int? Slot { get; set; }
    }
}
