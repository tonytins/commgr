Imports ArtManager.Models
Imports Xunit

Namespace ArtManager.Tests

    Public Class HashTests

        Const VERSION = "0.2"
        Const ORDER_TYPE = "order type:"

        Dim _art As Art

        <Theory>
        <InlineData("virtual", "Lupe Jacobson", "Lorem ipsum dolor sit amet", "Kasey.Goyette18")>
        Sub TestRequest(name As String, cust As String, desc As String, cont As String)

            _art = New Art() With {
                .Name = name,
                .Description = desc,
                .Custmer = New Customer() With {
                    .Name = cust,
                    .Contact = cont
                }
            }

            VersionCheck(_art.Version, VERSION)

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(name, cust, cont))

            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}")
            Debug.WriteLine(ArtUtils.AsJson(_art))

        End Sub

        <Theory>
        <InlineData("Tonga", "Alberta Mann", "Lorem ipsum dolor sit amet", "Chanel_McKenzie7",
                    "5458-2118-9194-8514", 42)>
        Sub TestCommission(name As String, cust As String, desc As String,
                           cont As String, payment As String, price As Decimal)

            _art = New Art() With {
                .Name = name,
                .Description = desc,
                .Custmer = New Customer() With {
                    .Name = cust,
                    .Contact = cont,
                    .Payment = payment
                },
                .Price = price
            }

            VersionCheck(_art.Version, VERSION)

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(name, price, cust, cont))
            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}")
            Debug.WriteLine(ArtUtils.AsJson(_art))

        End Sub

        <Theory>
        <InlineData("Synthesize", "Bessie Hettinger", "Jack.Torphy75", "31VLNZXfcpoA68wPRuWSdrmT3jv5k",
                    25, 1, 4)>
        Sub TestYCH(name As String, cust As String, cont As String, payment As String,
                    price As Decimal, ticket As Integer, slot As Integer)

            _art = New Art() With {
                .Name = name,
                .Custmer = New Customer() With {
                    .Name = cust,
                    .Contact = cont,
                    .Payment = payment
                },
                .Price = price,
                .Ticket = ticket,
                .Slot = slot
            }

            VersionCheck(_art.Version, VERSION)

            Assert.Equal(_art.Hash, ArtUtils.SearchHash(name, price, ticket, slot))
            Debug.WriteLine($"{Environment.NewLine}{_art.Name}, {ORDER_TYPE} {_art.Catagory}")
            Debug.WriteLine(ArtUtils.AsJson(_art))

        End Sub

    End Class
End Namespace

