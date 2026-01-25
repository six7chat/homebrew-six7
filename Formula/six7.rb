class Six7 < Formula
  desc "Secure peer-to-peer chatroom CLI built on Korium's adaptive networking fabric"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.22.tar.gz"
  sha256 "d5558cd419c8d46bdc958064cb97f963d1ea793866414c025906ec15033512ed"
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
