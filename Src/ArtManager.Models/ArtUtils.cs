/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using System.Text;
using System.Diagnostics;
using Serilog;
using Newtonsoft.Json;
using Bogus;

namespace ArtManager.Models
{
    internal class FillerData
    {
        public string Hash { get; set; }
    }

    public static class ArtUtils
    {
        static readonly JsonSerializerSettings _serializerSettings = new JsonSerializerSettings
        {
            NullValueHandling = NullValueHandling.Ignore,
        };

        /// <summary>
        /// Returns a Json string of the Art class, ignoring the
        /// null values.
        /// </summary>
        /// <param name="art"></param>
        /// <returns></returns>
        public static string AsJson(Art art)
        {
            return JsonConvert.SerializeObject(art, Formatting.Indented, _serializerSettings);
        }

        /// <summary>
        /// Returns a Json string of the Art class as an list,
        /// ignoring the null values.
        /// </summary>
        /// <param name="art"></param>
        /// <returns></returns>
        public static string AsJson(List<Art> art)
        {
            return JsonConvert.SerializeObject(art, Formatting.Indented, _serializerSettings);
        }

        /// <summary>
        /// Prints the Json data to the screen
        /// </summary>
        /// <param name="art"></param>
        public static void WriteJson(Art art)
        {
            if (Debugger.IsAttached)
                Debug.WriteLine(AsJson(art));
            else
                Console.WriteLine(AsJson(art));
        }

        /// <summary>
        /// Prints the Json data to the screen
        /// </summary>
        /// <param name="art"></param>
        public static void WriteJson(List<Art> art)
        {
            if (Debugger.IsAttached)
                Debug.WriteLine(AsJson(art));
            else
                Console.WriteLine(AsJson(art));
        }

        internal static string CalculateHash(Art art)
        {
            var filler = new Faker<FillerData>();
            filler.RuleFor(o => o.Hash, r => r.Random.Hash());

            using (var sha = SHA256.Create())
            {
                var input = string.Empty;

                switch (art.Catagory)
                {
                    case Catagory.Personal:
                        input = $"{art.Name}";
                        break;
                    case Catagory.Commission:
                        input = $"{art.Name}\\{art.Price}\\{art.Customer.Name}\\{art.Customer.Contact}";
                        break;
                    case Catagory.YCH:
                        input = $"{art.Name}\\{art.Price}\\{art.Ticket}\\{art.Slot}";
                        break;
                    case Catagory.Raffle:
                        input = $"{art.Name}\\{art.Ticket}\\{art.Slot}";
                        break;
                    case Catagory.Request:
                        input = $"{art.Name}\\{art.Customer.Name}\\{art.Customer.Contact}";
                        break;
                    case Catagory.Unknown:
                    default:
                        var mHash = filler.Generate();
                        return mHash.Hash;
                }

                var toBytes = Encoding.Unicode.GetBytes(input);
                var computeHash = sha.ComputeHash(toBytes);
                return Convert.ToBase64String(computeHash);
            }
        }

        public static string SearchHash(string name, string cust, string cont)
        {
            var art = new Art
            {
                Name = name,
                Customer = new Customer
                {
                    Name = cust,
                    Contact = cont,
                },
            };

            return CalculateHash(art);
        }

        public static string SearchHash(string name, decimal price, string cust, string cont)
        {
            var art = new Art
            {
                Name = name,
                Customer = new Customer
                {
                    Name = cust,
                    Contact = cont,
                },
                Price = price,
            };

            return CalculateHash(art);
        }

        public static string SearchHash(string name, decimal price, int ticket, int slot)
        {
            var art = new Art
            {
                Name = name,
                Ticket = ticket,
                Slot = slot,
                Price = price,
            };

            return CalculateHash(art);
        }

        public static string SearchHash(string name, int ticket, int slot)
        {
            var art = new Art
            {
                Name = name,
                Ticket = ticket,
                Slot = slot
            };

            return CalculateHash(art);
        }

        public static string SearchHash(string name)
        {
            var art = new Art
            {
                Name = name,
            };

            return CalculateHash(art);
        }

    }
}
