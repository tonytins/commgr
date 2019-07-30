using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using System.Text;
using ArtManager.Models;
using Newtonsoft.Json;

namespace ArtManager
{
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

        internal static string CalculateHash(Art art)
        {
            using (var sha = SHA256.Create())
            {
                var input = string.Empty;

                switch (art.Catagory)
                {
                    case Catagory.Commission:
                        input = $"{art.Name}/{art.Price}/{art.Custmer.Name}/{art.Custmer.Contact}";
                        break;
                    case Catagory.YCH:
                        input = $"{art.Name}/{art.Price}/{art.Ticket}/{art.Slot}";
                        break;
                    case Catagory.Request:
                    default:
                        input = $"{art.Name}/{art.Custmer.Name}/{art.Custmer.Contact}";
                        break;
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
                Custmer = new Customer
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
                Custmer = new Customer
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
    }
}
