// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using LiteDB;
using Newtonsoft.Json;
using Serilog;
using Bodkin.Serilog;

namespace ArtManager.Models
{
    public enum Catagory
    {
        Unknown,
        Personal,
        Request,
        Commission,
        YCH,
        Raffle,
    }

    public class Customer
    {
        public string Name { get; set; }
        public string Contact { get; set; }
        public string Payment { get; set; }
    }

    public class Art
    {

        public Art() { }

        public Art(bool unknown)
        {
            Unknown = unknown;
        }

        bool Unknown { get; set; }

        public Guid Id { get; } = Guid.NewGuid();
        public DateTime Timestamp { get; internal set; } = DateTime.Now;

        // Only used in data export
        [BsonIgnore]
        public string Version { get; } = "0.2";

        // Ignored in Json export since 0.1.1
        [JsonIgnore]
        public Catagory Catagory
        {
            get
            {
                if (!Slot.HasValue && !Ticket.HasValue
                    && !Price.HasValue && string.IsNullOrEmpty(Customer.Name))
                    return Catagory.Personal;
                else if (Slot.HasValue && Ticket.HasValue && Price.HasValue)
                    return Catagory.YCH;
                else if (Slot.HasValue && Ticket.HasValue)
                    return Catagory.Raffle;
                else if (!string.IsNullOrEmpty(Customer.Name) && Price.HasValue)
                    return Catagory.Commission;
                else if (Unknown)
                    return Catagory.Unknown;
                else
                    return Catagory.Request;
            }
        }

        public string Name { get; set; }
        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public string Status { get; set; }
        public decimal? Price { get; set; }
        public string Reference { get; set; }
        public string Description { get; set; }
        public Customer Customer { get; set; }

        public void Save(string dir, string filename)
        {
            var json = ArtUtils.AsJson(this);
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
            catch (IOException err)
            {
                SerilogHelper.LogException(err);
                throw new IOException(err.Message);
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
            var json = ArtUtils.AsJson(this);
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
            catch (IOException err)
            {
                SerilogHelper.LogException(err);
                throw new IOException(err.Message);
            }
        }

        public void LogModel()
        {
            switch (Catagory)
            {
                case Catagory.Personal:
                    Log.Information($"Personal: {Name}.");
                    break;
                case Catagory.Commission:
                    Log.Information($"Commission: {Name}, Price: {Price}, Customer: {Customer.Name}. Payment info on file.");
                    break;
                case Catagory.Request:
                    Log.Information($"Request: {Name}, Customer: {Customer.Name}.");
                    break;
                case Catagory.YCH:
                    Log.Information($"YCH: {Name}, Slot: {Slot}, Ticket: {Ticket}.");
                    break;
            }
        }

    }
}
