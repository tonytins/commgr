Imports Xunit
Imports ArtManager.Models

Namespace ArtManager.Tests

    Public Class BasicTests

        Const VERSION As String = "0.2"

        <Theory>
        <InlineData("virtual", "Lupe Jacobson", "Lorem ipsum dolor sit amet", "Kasey.Goyette18")>
        <InlineData("Tonga", "Alberta Mann", "Lorem ipsum dolor sit amet", "Chanel_McKenzie7")>
        Sub TestRequest(name As String, cust As String, desc As String, cont As String)

            Dim art As New Art() With {
                .Name = name,
                .Description = desc,
                .Custmer = New Customer() With {
                    .Name = cust,
                    .Contact = cont
                }
            }

            If art.Version IsNot VERSION Then
                Debug.WriteLine($"Version mismatch: {art.Version} is not {VERSION}")
            End If

            Assert.Equal(art.Name, name)
            Assert.Equal(art.Custmer.Name, cust)
            Assert.Equal(art.Description, desc)
            Assert.Equal(art.Custmer.Contact, cont)
        End Sub
    End Class
End Namespace

