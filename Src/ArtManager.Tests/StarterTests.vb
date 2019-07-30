Imports ArtManager.Models
Imports Xunit

Namespace ArtManager.Tests

    Public Class StarterTests

        Const VERSION As String = "0.2"
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

            Assert.Equal(_art.Name, name)
            Assert.Equal(_art.Custmer.Name, cust)
            Assert.Equal(_art.Description, desc)
            Assert.Equal(_art.Custmer.Contact, cont)

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

            Assert.Equal(_art.Name, name)
            Assert.Equal(_art.Custmer.Name, cust)
            Assert.Equal(_art.Description, desc)
            Assert.Equal(_art.Custmer.Contact, cont)
            Assert.Equal(_art.Custmer.Payment, payment)
            Assert.Equal(_art.Price, price)

        End Sub

        <Theory>
        <InlineData("Synthesize", "Bessie Hettinger", "Jack.Torphy75", "31VLNZXfcpoA68wPRuWSdrmT3jv5k", 25, 1, 4)>
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

            Assert.Equal(_art.Name, name)
            Assert.Equal(_art.Custmer.Name, cust)
            Assert.Equal(_art.Custmer.Contact, cont)
            Assert.Equal(_art.Custmer.Payment, payment)
            Assert.Equal(_art.Ticket, ticket)
            Assert.Equal(_art.Slot, slot)
            Assert.Equal(_art.Price, price)

        End Sub

    End Class
End Namespace

