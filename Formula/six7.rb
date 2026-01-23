class Six7 < Formula
  desc "Secure peer-to-peer chatroom CLI built on Korium's adaptive networking fabric"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.12.tar.gz"
  sha256 "8700cd690e8cfb612cb5d71121b07f03f57663f9bd1c117f7a20de5ceb460d79"
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
