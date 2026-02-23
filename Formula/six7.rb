class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.39"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.39"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.39/six7-0.7.39-arm64-apple-darwin.tar.gz"
      sha256 "f58ab84bedc7e790b62e32ef9aaf5e08bd58a7c3f68b60b36982a992fc401592"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
