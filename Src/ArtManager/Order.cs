// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Text.Json.Serialization;

namespace ArtManager
{
    class Order
    {
        Art NewOrder { get; set; }

        public Order(Art order)
        {
            NewOrder = order;
        }

        public string ToJson
        {
            get
            {
                var op = new JsonSerializerOptions()
                {
                    IgnoreNullValues = true,
                    WriteIndented = true,
                };

                return JsonSerializer.ToString(NewOrder, op);
            }
        }
    }
}
