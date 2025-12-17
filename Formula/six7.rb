class Six7 < Formula
  desc "Secure peer-to-peer chatroom CLI built on Korium's adaptive networking fabric"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.7.tar.gz"
  sha256 "b379b8697c4a73a7e7a2dcd5c9a90e4dcaea249f230a3b46510fbf7cc465f0aa"
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
