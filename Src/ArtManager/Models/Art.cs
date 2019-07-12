// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;

namespace ArtManager.Models
{
    class Customer
    {

        public string Name { get; set; }
        public string Contact { get; set; }
        public string Payment { get; set; }
    }

    class Art
    {

        public Guid Id { get; } = Guid.NewGuid();
        public DateTime Date { get; } = DateTime.Now;
        public string Version { get; } = "0.1.1";
        public string Name { get; set; }
        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public decimal? Price { get; set; }
        public string Reference { get; set; }
        public string Description { get; set; }
        public Customer Custmer { get; set; }
    }
}
