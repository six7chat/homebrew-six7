class Six7 < Formula
  desc "Secure peer-to-peer chatroom CLI built on Korium's adaptive networking fabric"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.13.tar.gz"
  sha256 "d8f3329f8d8bfbaab8c91c2cf10c2c3e485d5e1cb5bc7c96a09e767ea0b5ba81"
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
