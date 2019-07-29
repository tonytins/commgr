// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using System.IO;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;
using LiteDB;
using Newtonsoft.Json;

namespace ArtManager.Models
{
    public enum Catagory
    {
        Unknown,
        Request,
        Commission,
        YCH,
        Raffle
    }
    public class Customer
    {
        public string Name { get; set; }
        public string Contact { get; set; }
        public string Payment { get; set; }
    }

    public class Art
    {

        public string Hash
        {
            get
            {
                var sha = SHA256.Create();
                var toBytes = Encoding.Unicode.GetBytes($"{Timestamp}{Name}{Custmer.Name}");
                var computeHash = sha.ComputeHash(toBytes);
                sha.Dispose();
                return Convert.ToBase64String(computeHash);
            }
        }
        public DateTime Timestamp { get; } = DateTime.Now;

        // Only used in data export
        [BsonIgnore]
        public string Version { get; } = "0.2";

        // Ignored since 0.1.1
        [JsonIgnore]
        public Catagory Catagory { get; set; } = Catagory.Request;

        public string Name { get; set; }
        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public decimal? Price { get; set; }
        public string Reference { get; set; }
        public string Description { get; set; }
        public Customer Custmer { get; set; }

        public void Save(string dir, string filename)
        {
            var json = JsonConvert.SerializeObject(this, Formatting.Indented);
            var path = Path.Combine(dir, $"{filename}.artm");

            try
            {
                using (var writer = new StreamWriter(path))
                {
                    writer.Write(json);
                }

                if (Debugger.IsAttached)
                    Debug.WriteLine(json);
            }
            catch
            {
                throw new IOException();
            }
        }

        /// <summary>
        /// Writes a formatted Json file with the proper data provided
        /// by the database. Exention is not required.
        /// </summary>
        /// <param name="dir">Path to directory</param>
        /// <param name="filename">[filename].artm</param>
        /// <returns></returns>
        public async Task SaveAsync(string dir, string filename)
        {
            var json = JsonConvert.SerializeObject(this, Formatting.Indented);
            var path = Path.Combine(dir, $"{filename}.artm");

            try
            {
                using (var writer = new StreamWriter(path))
                {
                    await writer.WriteAsync(json);
                }

                if (Debugger.IsAttached)
                    Debug.WriteLine(json);
            }
            catch
            {
                throw new IOException();
            }
        }
    }
}
