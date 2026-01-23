class Six7 < Formula
  desc "Secure peer-to-peer chatroom CLI built on Korium's adaptive networking fabric"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.10.tar.gz"
  sha256 "102ca43855688bc9d18439c1446d1f73f5b638408268ef259b61adbfff68d008"
  license "MIT"
  head "https://github.com/six7chat/homebrew-six7.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
