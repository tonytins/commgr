// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;

namespace ArtManager
{
    static class Order
    {
        public static void Request(string name, string cust, string desc, bool debug)
        {
            if (debug)
            {
                Console.WriteLine($"Customer: {cust}{Environment.NewLine}Art: {name}{Environment.NewLine}Description: {desc}");
            }
            else
            {

            }
        }

        public static void Commission(string name, string cust, string desc, decimal price, string payment, bool debug)
        {
            if (debug)
            {
                Console.WriteLine($"Customer: {cust}{Environment.NewLine}Art: {name}{Environment.NewLine}Description: {desc}");
                Console.WriteLine($"Price: {price}{Environment.NewLine}Payment: {payment}");
            }
            else
            {

            }
        }
    }
}
