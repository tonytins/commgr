/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
using System;
using System.Collections.Generic;
using System.Diagnostics;
using ArtManager.Models;
using LiteDB;
using Serilog;
using ArtManager.Common;

namespace ArtManager.CLI
{
    class Order
    {
        Art Art { get; set; }

        public bool IsDebug { get; set; }

        readonly List<Art> _arts = new List<Art>();

        public Order() { }

        public Order(Art order)
        {
            Art = order;
        }

        public void DBInsert()
        {
            try
            {
                using (var db = new LiteDatabase(AppConfig.GetConfig.Database))
                {
                    var art = db.GetCollection<Art>(ArtmConsts.DB_COLUMN);
                    art.Insert(Art);
                }
            }
            catch (Exception err)
            {
                Log.Error(err.Message);
                throw new Exception(err.Message);
            }
        }

        public void DbListAll()
        {
            try
            {
                using (var db = new LiteDatabase(AppConfig.GetConfig.Database))
                {
                    var art = db.GetCollection<Art>(ArtmConsts.DB_COLUMN);
                    art.EnsureIndex(x => x.Id);
                    var query = art.Include(x => x.Id)
                        .Include(x => x.Customer)
                        .Include(x => x.Name)
                        .FindAll();

                    foreach (var q in query)
                        _arts.Add(q);

                    var orders = ArtUtils.AsJson(_arts);

                    if (IsDebug)
                        Console.WriteLine(orders);
                    else if (Debugger.IsAttached)
                        Debug.WriteLine(orders);
                }
            }
            catch (Exception err)
            {
                Log.Error(err.Message);
                throw new Exception(err.Message);
            }
        }
    }
}